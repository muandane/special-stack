<script>
	/*const backgroundUrl = import.meta.env.VITE_EPIC_GIF;
	const imageText = import.meta.env.VITE_EPIC_NAME;*/
	// const backgroundUrl = import.meta.env.VITE_EPIC_GIF;
	// const imageText = import.meta.env.VITE_EPIC_NAME;
	import 'dotenv/config';
	import LatencyCounter from './HUD.svelte';
	const backgroundUrl = process.env.PUBLIC_EPIC_GIF;
	const imageText = process.env.PUBLIC_EPIC_NAME;
</script>


<div class="container">
	<p>{imageText}</p>
</div>

<div id="main">
	<div class="stars">
		<span class="s"></span>
		<span class="s"></span>
		<span class="s"></span>
		<span class="s"></span>
		<span class="s"></span>
		<span class="m"></span>
		<span class="m"></span>
		<span class="m"></span>
		<span class="m"></span>
		<span class="m"></span>
		<span class="l"></span>
		<span class="l"></span>
		<span class="l"></span>
		<span class="l"></span>
		<span class="l"></span>
	</div>
	<div id="piece">
		<div id="holder" style="background: url({backgroundUrl}); background-size: cover;">
			<div id="painting">
				<div id="original"></div>
			</div>
		</div>
		<div id="frame"></div>
	</div>
</div>

<LatencyCounter {backgroundUrl} />
<style>
	:root {
		--min-fs: 0.5;
		--max-fs: 2;
		--min-vw: 10;
		--max-vw: 80;

		--min-fs-rem: calc(var(--min-fs) * 1rem);
		--max-fs-rem: calc(var(--max-fs) * 1rem);
		--min-vw-rem: calc(var(--min-vw) * 0.1vw);

		--slope: calc((var(--max-fs) - var(--min-fs)) * (100vw - var(--min-vw-rem)) / (var(--max-vw) - var(--min-vw)));

		--font-size-container-p: clamp(var(--min-fs-rem), var(--min-fs-rem) + var(--slope), var(--max-fs-rem));
	}

	#main {
		display: flex;
		flex-direction: column;
		align-items: center;
		align-content: center;
	}

	#piece {
		box-shadow: 0 0 20px 0 rgba(0, 0, 0, 0.25);
		cursor: pointer;
		height: clamp(30vh, 50vw, 50vh);
		max-height: 820px;
		position: relative;
		width: calc(clamp(30vh, 50vw, 50vh) * 0.774);
	}

	#frame {
		background: url(https://s3-us-west-2.amazonaws.com/s.cdpn.io/141041/banksy-frame.png);
		background-size: cover;
		height: 100%;
		position: relative;
		width: 100%;
		/* border-radius: 8px; */

		&::after {
			position: absolute;
			content: '';
			top: 40px;
			left: 0;
			right: 0;
			z-index: -1;
			height: 100%;
			width: 100%;
			transform: scale(0.9) translateZ(0);
			filter: blur(45px);
			background: linear-gradient(
				to left,
				#ff5770,
				#e4428d,
				#c42da8,
				#9e16c3,
				#6501de,
				#9e16c3,
				#c42da8,
				#e4428d,
				#ff5770
			);
			background-size: 200% 200%;
			animation: animateGlow 1.25s linear infinite;
		}
		box-shadow: 0 0 0 10px rgba(0, 0, 0, 0.01);
	}

	@keyframes animateGlow {
		0% {
			background-position: 0% 50%;
		}
		100% {
			background-position: 200% 50%;
		}
	}

	#holder {
		background: url(https://hampter.io/hampter.gif);
		background-size: cover;
		height: calc(clamp(30vh, 50vw, 50vh) * 0.79);
		align-content: center;
		left: 0.9%;
		position: absolute;
		top: 5.4%;
		width: calc(clamp(30vh, 50vw, 50vh) * 0.774 * 0.817);
	}

	#painting {
		height: 100%;
		position: absolute;
		transform: translateY(0%);
		width: 100%;
	}

	#original {
		height: 100%;
		position: absolute;
		top: 0;
		width: 100%;
	}

	.container {
		position: relative;
		width: clamp(350px, 50%, 450px);
		background: transparent;
		margin-bottom: 20px;
		display: flex;
		justify-content: center;
		align-items: center;
		flex-direction: column;
		& p {
			padding: 0;
			margin: 0;
			font-size: var(--font-size-container-p);
			line-height: 1em;
			font-weight: bold;
			color: transparent;
			background: #d8c5ff;
			-webkit-background-clip: text;
			background-clip: text;
		}
	}


	.stars {
		position: absolute;
		left: 0;
		top: 0;
		width: 100%;
		height: 100%;
		& span {
			position: relative;
			&:before {
				content: '';
				position: absolute;
				width: 10px;
				height: 10px;
				transform: scale(0.2);
			}
			&.s:before {
				box-shadow: 20px 20px #ffffff;
			}
			&.m:before {
				box-shadow:
					20px 10px #fff176,
					10px 20px #f9e74a,
					20px 20px #ffffff,
					30px 20px #d9c830,
					20px 30px #fff176;
			}
			&.l:before {
				box-shadow: 
					/*0段目*/
					20px 0px #357bfd,
					/*1段目*/ 20px 10px #fff176,
					/*2段目*/ 0px 20px #35fdad,
					10px 20px #fff176,
					20px 20px #ffffff,
					30px 20px #fff176,
					40px 20px #35d8fd,
					/*3段目*/ 20px 30px #fff176,
					/*4段目*/ 20px 40px #fdd835;
			}

			&:nth-child(1) {
				left: 10%;
				top: 10%;
				animation: star-anim 8s 0s infinite;
			}
			&:nth-child(2) {
				left: 80%;
				top: 20%;
				animation: star-anim 6s 1s infinite;
			}
			&:nth-child(3) {
				left: 60%;
				top: 40%;
				animation: star-anim 5s 2s infinite;
			}
			&:nth-child(4) {
				left: 20%;
				top: 70%;
				animation: star-anim 7s 3s infinite;
			}
			&:nth-child(5) {
				left: 80%;
				top: 80%;
				animation: star-anim 7s 4s infinite;
			}

			&:nth-child(6) {
				left: 40%;
				top: 10%;
				animation: star-anim 4s 0s infinite;
			}
			&:nth-child(7) {
				left: 10%;
				top: 50%;
				animation: star-anim 7s 1s infinite;
			}
			&:nth-child(8) {
				left: 50%;
				top: 50%;
				animation: star-anim 8s 2s infinite;
			}
			&:nth-child(9) {
				left: 90%;
				top: 90%;
				animation: star-anim 5s 3s infinite;
			}
			&:nth-child(10) {
				left: 80%;
				top: 50%;
				animation: star-anim 7s 4s infinite;
			}

			&:nth-child(11) {
				left: 90%;
				top: 10%;
				animation: star-anim 5s 0s infinite;
			}
			&:nth-child(12) {
				left: 10%;
				top: 80%;
				animation: star-anim 6s 1s infinite;
			}
			&:nth-child(13) {
				left: 30%;
				top: 20%;
				animation: star-anim 7s 2s infinite;
			}
			&:nth-child(14) {
				left: 60%;
				top: 90%;
				animation: star-anim 8s 3s infinite;
			}
			&:nth-child(15) {
				left: 80%;
				top: 50%;
				animation: star-anim 7s 4s infinite;
			}
		}
	}

	@keyframes star-anim {
		0% {
			opacity: 0;
		}
		5% {
			opacity: 1;
		}
		10% {
			opacity: 0;
		}
		55% {
			opacity: 0;
		}
		60% {
			opacity: 1;
		}
		95% {
			opacity: 0;
		}
		100% {
			opacity: 1;
		}
	}

</style>