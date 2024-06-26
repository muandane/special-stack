<script>
  import { onMount } from 'svelte';
  /** @type {string} */
  export let backgroundUrl;

  /** @type {number | null} */
  let latency = null;

  /** @type {number | null} */
  let serverLatency = null;

  /** @type {string | null} */
  let serverLocation = null;

  async function measureLatency() {
    try {
      // Measure latency for backgroundUrl
      const startTime = performance.now();
      const response = await fetch(backgroundUrl);
      const endTime = performance.now();
      latency = endTime - startTime;

      if (!response.ok) {
        throw new Error('Failed to fetch background URL');
      }

      // Measure latency for the server
      const serverStartTime = performance.now();
      const serverEndTime = performance.now();
      serverLatency = serverEndTime - serverStartTime;
      const serverResponse = await fetch('https://worldtimeapi.org/api/ip');

      if (!serverResponse.ok) {
        throw new Error('Failed to fetch server data');
      }

      const serverData = await serverResponse.json();
      serverLocation = serverData.timezone;
    } catch (error) {
      console.error('Error measuring latency:', error);
      latency = serverLatency = -1; // Use -1 to indicate an error
      serverLocation = 'Error';
    }
  }

  onMount(measureLatency);
</script>

<style>
  :root {
    --min-l-fs: 0.5;
    --max-l-fs: 2.5;
    --min-s-fs: 0.2;
    --max-s-fs: 2.2;
    --min-vw: 10;
    --max-vw: 40;

    --min-fs-l-rem: calc(var(--min-l-fs) * 0.8rem);
    --max-fs-l-rem: calc(var(--max-l-fs) * 0.8rem);
    --min-fs-s-rem: calc(var(--min-s-fs) * 0.5rem);
    --max-fs-s-rem: calc(var(--max-s-fs) * 0.5rem);
    --min-vw-rem: calc(var(--min-vw) * 0.1vw);

    --l-slope: calc((var(--max-l-fs) - var(--min-l-fs)) * (50vw - var(--min-vw-rem)) / (var(--max-vw) - var(--min-vw)));
    --s-slope: calc((var(--max-s-fs) - var(--min-s-fs)) * (30vw - var(--min-vw-rem)) / (var(--max-vw) - var(--min-vw)));

    --font-size-large: clamp(var(--min-fs-l-rem), var(--min-fs-l-rem) + var(--l-slope), var(--max-fs-l-rem));
    --font-size-small: clamp(var(--min-fs-s-rem), var(--min-fs-s-rem) + var(--s-slope), var(--max-fs-s-rem));
  }

  .latency-counter {
    font-family: 'Courier New', Courier, monospace;
    color: #00ff00;
    background-color: rgba(5, 47, 46, 0.434);
    padding: 10px;
    border-radius: 5px;
    position: fixed;
    top: 20px;
    left: 20px;
    z-index: 1000;
    width: clamp(140px, 25vw, 350px); /* Adjusted for better responsiveness */
    height: auto; /* Auto height to fit content */
    max-height: 150px;
    overflow-wrap: break-word;
    word-break: break-all;
    display: flex;
    flex-direction: column;
  }

  .latency-counter div {
    margin: 5px 0;
    white-space: nowrap;
    overflow-wrap: break-word;
    word-break: break-all;
  }

  .latency-counter .large {
    font-size: var(--font-size-large);
  }

  .latency-counter .small {
    font-size: var(--font-size-small);
  }
</style>

<div class="latency-counter">
  <div class="large">Latency Counter</div>
  <div class="small">Client: {latency !== null ? latency.toFixed(2) + ' ms' : 'Calculating...'}</div>
  <div class="small">Server Location: {serverLocation || 'Fetching...'}</div>
  <div class="small">Server: {serverLatency !== null && serverLatency !== -1 ? serverLatency.toFixed(2) + ' ms' : 'Calculating...'}</div>
</div>
