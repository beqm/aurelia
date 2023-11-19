<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import RecordRTC from "recordrtc";

  let recorder;
  let audioData;
  let audioBlob;
  let talking = false;

  const startTalk = () => {
    if (talking == true) return;
    talking = true;
    startRecording();
  };

  const stopTalk = () => {
    if (talking == false) return;
    talking = false;
    stopRecording();
  };

  const startRecording = async () => {
    await navigator.mediaDevices
      .getUserMedia({ audio: true })
      .then((stream) => {
        console.log("Microphone permission granted!");
        recorder = RecordRTC(stream, { type: "audio" });
        recorder.startRecording();
      })
      .catch((error) => {
        console.error("Error accessing microphone:", error);
      });
  };

  const stopRecording = () => {
    recorder.stopRecording(() => {
      audioBlob = recorder.getBlob();

      const reader = new FileReader();
      reader.onloadend = () => {
        const arrayBuffer = reader.result as ArrayBuffer;
        const uint8Array = new Uint8Array(arrayBuffer);
        audioData = Array.from(uint8Array);

        console.log(uint8Array);
        console.log(audioData);

        invoke("save_audio", { data: Array.from(uint8Array) }).then(
          (result) => {
            console.log(result);
          }
        );
      };

      reader.readAsArrayBuffer(audioBlob);
    });
  };

  const downloadAudio = () => {
    if (audioBlob) {
      const blob = new Blob([audioBlob], { type: "audio/wav" });

      // Create a download link
      const downloadLink = document.createElement("a");
      downloadLink.href = URL.createObjectURL(blob);
      downloadLink.download = "sample.wav";

      // Append the link to the body
      document.body.appendChild(downloadLink);

      // Trigger the download
      downloadLink.click();

      // Remove the link from the body
      document.body.removeChild(downloadLink);
    }
  };
</script>

<div class="flex w-full justify-center active:scale-90 duration-200 mt-10">
  {#if !talking}
    <button on:click={startTalk} class="bg-blue-400 max-w-[150px] p-2"
      >Start Talking</button
    >
  {:else}
    <button on:click={stopTalk} class="bg-red-400 max-w-[150px] p-2"
      >Stop Talking</button
    >
  {/if}

  {#if audioBlob}
    <button on:click={downloadAudio} class="ml-4 bg-blue-400 max-w-[150px] p-2">
      Download Audio
    </button>
  {/if}
</div>
