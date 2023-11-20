<script lang="ts">
  import "./app.postcss";
  import Voice from "./lib/components/Voice.svelte";
  import Header from "./lib/components/Header.svelte";
  import Prompt from "./lib/components/Prompt.svelte";
  import { invoke } from "@tauri-apps/api/tauri";
  import Alarm from "./lib/components/Alarm.svelte";
  import Footer from "./lib/components/Footer.svelte";
  import { onMount } from "svelte";

  let justPrompted = false;
  let supervisioning = false;

  $: {
    if (supervisioning == true) {
      setInterval(async () => {
        if (supervisioning == true) {
          let result: any = await invoke("supervision");
          console.log(result);
          if (result.status == "not-focus") {
            justPrompted = true;
          }
        }
      }, 30000);
    }
  }

  onMount(async () => {
    let result: any = await invoke("check_reminder");
    if (result.status == "202") {
      justPrompted = true;
    }
  });
</script>

<main
  class="m-0 p-0 h-screen w-screen flex flex-col overflow-hidden bg-[#09060f]"
>
  <Header />
  <Voice bind:justPrompted />
  <!-- <Talk /> -->
  <div class="mt-8 flex h-[35px] items-center justify-center">
    <Prompt
      bind:supervisioning
      on:enter={() => {
        justPrompted = true;
      }}
    />
    <!-- <Footer /> -->
  </div>
</main>
<Alarm
  on:time={() => {
    justPrompted = true;
  }}
/>
