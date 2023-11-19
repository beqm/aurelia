<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { createEventDispatcher } from "svelte";

  const dispatch = createEventDispatcher();
  let prompt = "";

  const onEnter = async () => {
    prompt = "";
    await invoke("prompt_response", { prompt: prompt });
    dispatch("enter", {
      justPrompted: true,
    });
  };
</script>

<div class="w-full flex justify-center">
  <div class="w-[70%] bg-[#223757] relative">
    <input
      class="w-full border bg-[#223757] h-10 placeholder:p-4 rounded-md"
      type="text"
      placeholder="Escreva algo.."
      bind:value={prompt}
    />
    <button
      on:click={onEnter}
      class="bg-[#233758] p-2 pl-4 pr-4 rounded-md absolute right-0"
      ><svg
        fill="currentColor"
        xmlns="http://www.w3.org/2000/svg"
        height="1.2em"
        viewBox="0 0 256 512"
        ><!--! Font Awesome Free 6.4.2 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license (Commercial License) Copyright 2023 Fonticons, Inc. --><path
          d="M246.6 278.6c12.5-12.5 12.5-32.8 0-45.3l-128-128c-9.2-9.2-22.9-11.9-34.9-6.9s-19.8 16.6-19.8 29.6l0 256c0 12.9 7.8 24.6 19.8 29.6s25.7 2.2 34.9-6.9l128-128z"
        /></svg
      ></button
    >
  </div>
</div>
