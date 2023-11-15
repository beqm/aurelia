<script lang="ts">
  import "./app.postcss";
  import Voice from "./lib/components/Voice.svelte";
  import Header from "./lib/components/Header.svelte";
  import Talk from "./lib/components/Talk.svelte";
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount } from "svelte";
  import Footer from "./lib/components/Footer.svelte";

  // let audioElement;
  // let audioUrl = "../aurelia/voice/startup/aurelia.wav";

  // function playAudio() {
  //   audioElement.play();
  // }

  // onMount(() => {
  //   // Automatically play the audio when the component is mounted
  //   playAudio();
  // });
  let mediaRecorder;
  let recordedChunks = [];
  let audioUrl;

  let x = false;

  function doit() {
    console.log(recordedChunks);
    if (!x) {
      x = !x;
      startRecording();
    } else {
      console.log("stopped");
      stopRecording();
      x = !x;
    }
  }

  async function startRecording() {
    const stream = await navigator.mediaDevices.getUserMedia({ audio: true });

    mediaRecorder = new MediaRecorder(stream);

    mediaRecorder.ondataavailable = (event) => {
      if (event.data.size > 0) {
        recordedChunks.push(event.data);
      }
    };

    mediaRecorder.onstop = async () => {
      const audioBlob = new Blob(recordedChunks, { type: "audio/wav" });
      audioUrl = URL.createObjectURL(audioBlob);
      const arrayBuffer = await audioBlob.arrayBuffer();
      const uint8Array = new Uint8Array(arrayBuffer);
    };

    recordedChunks = [];

    mediaRecorder.start();
  }

  async function stopRecording() {
    mediaRecorder.stop();
    const audioBlob = new Blob(recordedChunks, { type: "audio/wav" });
    const arrayBuffer = await audioBlob.arrayBuffer();
    const uint8Array = new Uint8Array(arrayBuffer);

    saveAudio(uint8Array);
  }

  async function saveAudio(uint8Array) {
    console.log(uint8Array);
    const result = await invoke("save_audio", { data: uint8Array });
    console.log(result);
  }
</script>

<main class="m-0 p-0 h-screen w-screen flex flex-col bg-[#09060f]">
  <Header />
  <Voice />
  <Talk />
  <Footer />
</main>
