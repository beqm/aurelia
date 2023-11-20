<script>
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/tauri";
  import { createEventDispatcher } from "svelte";

  const dispatch = createEventDispatcher();
  // Function to check the alarm using Tauri command
  const checkAlarm = async () => {
    try {
      let state = false;
      if (state == false) {
        const result = await invoke("check_alarm");
        if (result.status == "202") {
          dispatch("time", {
            justPrompted: true,
          });
          state = true;
        }
      }
    } catch (error) {
      console.error("Error checking alarm:", error);
    }
  };

  // Call the Tauri command every second
  onMount(() => {
    const intervalId = setInterval(checkAlarm, 1000);

    // Cleanup on component destroy
    return () => clearInterval(intervalId);
  });
</script>
