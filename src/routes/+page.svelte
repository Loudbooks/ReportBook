<script lang="ts">
    import Shield from "./glyph/Shield.svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { preferredUsername, report } from "$lib/stores";
    import { goto } from "$app/navigation";
    import { onMount } from "svelte";

    let button: HTMLButtonElement;
    let usernameInput: HTMLInputElement;

    async function requestReport() {
      button.innerHTML = "Working...";

      setTimeout(async () => {
        report.set(await invoke("request_report"))
        await goto("/report")
      }, 50);
    }

    async function checkUsernameInput() {
      button.disabled = usernameInput.value.length <= 0;

      $preferredUsername = usernameInput.value;
    }

    onMount(() => {
      if ($preferredUsername) {
        usernameInput.value = $preferredUsername;
      }

      checkUsernameInput();
    });
</script>

<content-container>
    <title-container>
        <h1>ReportBook</h1>
        <p id="description">Simplistic automated diagnostic tool.</p>
    </title-container>
    <spacer/>
    <items>
        <input-container>
            <input placeholder="Preferred Username" on:input={checkUsernameInput} on:keyup={
                (event) => {
                    if (event.key === "Enter") {
                        requestReport();
                    }
                }
            } bind:this={usernameInput}>
        </input-container>
        <button id="submit" on:click={requestReport} bind:this={button}>
            Continue
        </button>
        <information>
            <h2>Collected Information</h2>
            <p class="info-description">
                ReportBook collects the following information:
            </p>
            <ul>
                <li>
                    Hardware information (CPU, GPU, RAM)
                </li>
                <li>
                    Operating System
                </li>
                <li>
                    Running Processes
                </li>
                <li>
                    Installed Apps
                </li>
                <li>
                    Hosts File
                </li>
            </ul>
            <note>
                <shield>
                    <Shield/>
                </shield>
                <p class="info-description"><strong>Note:</strong> Your true device username will be automatically removed in the diagnostic report to protect your privacy.</p>
            </note>
        </information>
    </items>
</content-container>

<style lang="scss">
    content-container {
      display: flex;
      flex-direction: column;
      align-items: center;
      justify-content: center;
      width: 100%;
      height: 100%;
      position: relative;
    }

    h1 {
      text-align: center;
      font-family: "DM Sans", sans-serif;
      font-weight: 850;
      font-size: 7rem;
      margin: 0;
      letter-spacing: -3.5px;
    }

    #description {
      text-align: center;
      font-family: "DM Sans", sans-serif;
      font-weight: 400;
      font-size: 1.5rem;
      color: gray;
      margin: 0;
      letter-spacing: -1px;
    }

    spacer {
      height: 8rem;
    }

    items {
      display: flex;
      flex-direction: column;
      height: 100%;
      gap: 20px;
    }

    input {
      all: unset;
      border: 2.5px solid #BCBCBC;
      width: 400px;
      height: 18px;
      border-radius: 15px;
      font-family: "DM Sans", sans-serif;
      font-size: 13px;
      font-weight: 400;
      outline: none;
      padding: 10px;
      color: #8E8E8E;

      &::placeholder {
        color: #BCBCBC;
      }
    }

    button {
      all: unset;
      border: 2.5px solid black;
      width: 400px;
      height: 18px;
      border-radius: 15px;
      font-family: "DM Sans", sans-serif;
      font-size: 16px;
      outline: none;
      padding: 10px;
      background-color: black;
      text-align: center;
      font-weight: 900;
      color: white;
      transition: transform 0.3s ease, opacity 0.3s ease;

      &:disabled,
      &[disabled]{ 
        opacity: 0.8;
        
        &:hover {
          cursor: not-allowed;
        }

        &:active {
          transform: scale(1);
        }
      }

      &:hover {
        cursor: pointer;
      }

      &:active {
        transform: scale(0.98);
      }
    }

    information {
      background-color: #F1F1F1;
      padding: 20px;
      border-radius: 15px;
      width: 385px;
    }

    h2 {
      margin: 0;
      font-family: "DM Sans", sans-serif;
      font-size: 22px;
      font-weight: 1000;
      letter-spacing: -0.7px;
    }

    ul, .info-description {
      letter-spacing: -0.2px;
      font-family: "DM Sans", sans-serif;
      font-weight: 400;
      color: #4B4B4B;
      font-size: 13px;
    }

    .info-description {
      margin-top: 8px;
    }

    ul {
      padding-left: 15px;
    }

    note {
      display: flex;
      align-items: flex-start;
      justify-content: flex-start;
      height: auto;
      align-content: flex-start;
      gap: 8px;
      transform: translateX(-6px);

      shield {
        height: 30px;
        width: 30px;

      }

      p {
        margin: 0 !important;
      }
    }
</style>