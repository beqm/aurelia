<script lang="ts">
  import "./app.postcss";
  import Voice from "./lib/components/Voice.svelte";
  import Header from "./lib/components/Header.svelte";
  import Prompt from "./lib/components/Prompt.svelte";

  import Footer from "./lib/components/Footer.svelte";

  let justPrompted = false;
</script>

<main
  class="m-0 p-0 h-screen w-screen flex flex-col overflow-hidden bg-[#09060f]"
>
  <Header />
  <Voice bind:justPrompted />
  <!-- <Talk /> -->
  <div class="mt-8 flex h-[35px] items-center justify-center">
    <Prompt
      on:enter={() => {
        justPrompted = true;
      }}
    />
    <Footer />
  </div>
</main>
