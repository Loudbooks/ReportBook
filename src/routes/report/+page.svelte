<script lang="ts">
    import axios from 'axios';
    import Upload from "../glyph/Upload.svelte";
    import { preferredUsername, report } from "$lib/stores";
    import Copy from '../glyph/Copy.svelte';
    import Check from '../glyph/Check.svelte';
    import Loading from '../glyph/Loading.svelte';
    import { goto } from '$app/navigation';

    let uploadButton: HTMLButtonElement;

    let url = "https://pastebook.dev/api/upload"

    let upload = false;
    let copy = false;

    let reportUrl = "";

    async function submit(_: MouseEvent) {
      if (reportUrl) {
        window.__TAURI__.shell.open(reportUrl);
      }

      if (upload) return;
      
      upload = true;
      const title = `${$preferredUsername}'s ReportBook`;

      try {
        const result = await axios.post(url, $report, {
          headers: {
            'Content-Type': 'text/plain',
            'title': title,
            'reportbook': 'true',
            'unlisted': 'true'
          }
        });

        reportUrl = `https://pastebook.dev/p/${result.data}`;

        uploadButton.innerText = reportUrl;
        copyToClipboard();
      } catch (error) {
        uploadButton.innerText = "Failed to upload. Try again?";

        upload = false;
      }
    }

    function copyToClipboard() {
      navigator.clipboard.writeText(reportUrl);
      copy = true;

      setTimeout(() => {
        copy = false;
      }, 5000);
    }

    let uploadActive = false;

    function handleMouseDown() {
        uploadActive = true;
    }

    function handleMouseUp() {
        uploadActive = false;
    }

    function goBack() {
      $report = "";
      goto("/");
    }
</script>

<content>
  <title-container>
    <h1>{$preferredUsername}'s ReportBook</h1>
    <button on:click={goBack}>Back</button>
  </title-container>

  <report-container>
    <p id="report-content">
      {#each $report.split("\n") as line}
        <line-container>
          {#if !line}
            <p id="spacer"> </p>
          {/if}
          <p id="line">{line}</p>
        </line-container>
      {/each} 
    </p>
  </report-container>
  <footer>
    <div class="upload-container {uploadActive ? 'active' : ''}">
      {#if reportUrl}
        {#if copy}
          <button class="copy" id="copy-check">
            <Check/>
          </button>
        {:else}
          <button class="copy" on:click={copyToClipboard}>
            <Copy/>
          </button>
        {/if}
      {:else}
        <button class="copy">
          {#if (upload && !copy)}
            <Loading/>
          {:else}
            <Upload/>
          {/if}
        </button>
      {/if}
      <button id="upload"
          on:mousedown={handleMouseDown}
          on:mouseup={handleMouseUp}
          on:mouseleave={handleMouseUp}
          on:click={submit}
          bind:this={uploadButton}>
          Upload to PasteBook
      </button>
    </div>
  </footer>
</content>

<style lang="scss">
  content {
    display: flex;
    flex-direction: column;
    height: 100vh;
  }

  h1 {
    padding-left: 10px;
    font-family: "DM Sans", sans-serif;
    font-weight: 1000;
    margin-top: 10px;
    margin-bottom: 0;
  }

  title-container {
    display: flex;
    justify-content: space-between;
    align-items: flex-end;
    margin: 0 15px;

    button {
      all: unset;
      color: gray;
      font-family: "DM Sans", sans-serif;
      font-size: 15px;
      text-decoration: underline;
      margin-top: 10px;
      transform: translateY(-5px);
      padding-right: 10px;

      &:hover {
        cursor: pointer;
      }

      &:active {
        transform: translateY(-5px) scale(0.95);
      }

      transition: all 0.2s;
    }
  }

  report-container {
    background-color: #F1F1F1;
    font-family: "JetBrains Mono", monospace;
    border-radius: 15px;
    flex-grow: 1;
    overflow: hidden;
    margin: 10px 10px 5px;
    margin-top: 5px;

    width: calc(100% - 20px);
    display: flex;
    flex-direction: column;

    float: left;
  }

  #report-content {
    font-family: "JetBrains Mono", monospace;
    margin: 0;
    padding: 20px;
    height: 100%;
    overflow-y: auto;
    font-size: 13px;
  }

  footer {
    font-family: "DM Sans", sans-serif;
    margin: 0 15px 8px;
    display: flex;
    align-items: center;
    gap: 5px;

    #upload {
      all: unset;
      text-align: left;
      color: gray;
      font-size: 15px;
      text-decoration: underline;

      &:hover {
        cursor: pointer;
      }

      transition: all 0.2s;
    }

    .upload-container {
      display: flex;
      align-items: center;
      gap: 5px;
      transition: transform 0.2s;
    }

    .upload-container.active {
      transform: scale(0.97);
    }

    .copy {
      all: unset;
      display: flex;
      align-items: center;

      &:hover {
        cursor: pointer;
      }
    }

    #copy-check {
      &:hover {
        cursor: default;
      }
    }
  }

  #line {
    all: unset;
    display: block;
    margin: 0;
    padding: 0;
    text-wrap: nowrap;
    white-space: pre
  }

  #spacer {
    display: block;
  }
</style>
