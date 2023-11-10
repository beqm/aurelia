<script lang="ts">
  import "./app.postcss";
  import Voice from "./lib/components/Voice.svelte";
  import Header from "./lib/components/Header.svelte";
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
</script>

<main class="m-0 p-0 h-screen w-screen flex flex-col bg-[#09060f]">
  <Header />
  <Voice />
  <Footer />
</main>
