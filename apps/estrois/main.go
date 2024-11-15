package main

import (
	"bytes"
	"context"
	"fmt"
	"net"
	"net/http"
	"os"
	"time"

	"github.com/aws/aws-sdk-go-v2/aws"
	"github.com/aws/aws-sdk-go-v2/config"
	"github.com/aws/aws-sdk-go-v2/service/s3"
	"github.com/gin-gonic/gin"
)

var s3Client *s3.Client

func init() {
	region := getEnv("AWS_REGION", "us-east-1")

	cfg, err := config.LoadDefaultConfig(context.TODO(),
		config.WithRegion(region),
		config.WithHTTPClient(&http.Client{
			Timeout:   30 * time.Second,
			Transport: getS3TransportWithSigV4(),
		}),
	)
	if err != nil {
		panic(fmt.Sprintf("Failed to load AWS config: %v", err))
	}

	s3Client = s3.NewFromConfig(cfg)
}

func getEnv(key, defaultValue string) string {
	value := os.Getenv(key)
	if value == "" {
		return defaultValue
	}

	return value
}

func main() {
	r := gin.Default()
	r.GET("/objects/:bucket/*key", getObject)
	r.PUT("/objects/:bucket/*key", putObject)
	r.DELETE("/objects/:bucket/*key", deleteObject)
	r.HEAD("/objects/:bucket/*key", headObject)
	r.Run()
}

func getObject(c *gin.Context) {
	bucket := c.Param("bucket")
	key := c.Param("key")

	resp, err := s3Client.GetObject(context.TODO(), &s3.GetObjectInput{
		Bucket: aws.String(bucket),
		Key:    aws.String(key),
	})
	if err != nil {
		c.AbortWithError(http.StatusInternalServerError, err)
		return
	}
	defer resp.Body.Close()

	c.DataFromReader(http.StatusOK, aws.ToInt64(resp.ContentLength), aws.ToString(resp.ContentType), resp.Body, nil)
}

func putObject(c *gin.Context) {
	bucket := c.Param("bucket")
	key := c.Param("key")

	body, err := c.GetRawData()
	if err != nil {
		c.AbortWithError(http.StatusInternalServerError, err)
		return
	}

	_, err = s3Client.PutObject(context.TODO(), &s3.PutObjectInput{
		Bucket: aws.String(bucket),
		Key:    aws.String(key),
		Body:   bytes.NewReader(body),
	})
	if err != nil {
		c.AbortWithError(http.StatusInternalServerError, err)
		return
	}

	c.Status(http.StatusOK)
}

func deleteObject(c *gin.Context) {
	bucket := c.Param("bucket")
	key := c.Param("key")

	_, err := s3Client.DeleteObject(context.TODO(), &s3.DeleteObjectInput{
		Bucket: aws.String(bucket),
		Key:    aws.String(key),
	})
	if err != nil {
		c.AbortWithError(http.StatusInternalServerError, err)
		return
	}

	c.Status(http.StatusNoContent)
}

func headObject(c *gin.Context) {
	bucket := c.Param("bucket")
	key := c.Param("key")

	resp, err := s3Client.HeadObject(context.TODO(), &s3.HeadObjectInput{
		Bucket: aws.String(bucket),
		Key:    aws.String(key),
	})
	if err != nil {
		c.AbortWithError(http.StatusInternalServerError, err)
		return
	}

	c.Header("Content-Type", aws.ToString(resp.ContentType))
	c.Header("Content-Length", fmt.Sprintf("%d", aws.ToInt64(resp.ContentLength)))
	c.Status(http.StatusOK)
}

func getS3TransportWithSigV4() *http.Transport {
	const timeout = 30 * time.Second

	dialer := &net.Dialer{
		Timeout:   timeout,
		KeepAlive: timeout,
		DualStack: true,
	}

	return &http.Transport{
		Proxy:                 http.ProxyFromEnvironment,
		DialContext:           dialer.DialContext,
		ForceAttemptHTTP2:     true,
		MaxIdleConns:          100,
		IdleConnTimeout:       90 * time.Second,
		TLSHandshakeTimeout:   10 * time.Second,
		ExpectContinueTimeout: 1 * time.Second,
	}
}
