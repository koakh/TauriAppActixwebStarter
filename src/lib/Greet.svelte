<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";

  let name = "Koakh";
  let greetMsg = "";

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    greetMsg = await invoke("greet", { name });
  }
  async function random() {
    greetMsg = await invoke("random", {});
  }
</script>

<div>
  <form class="row" on:submit|preventDefault={greet}>
    <input id="greet-input" placeholder="Enter a name..." bind:value={name} />
    <button type="submit">Greet</button>
  </form>
  <button type="submit" on:click={random}>Random</button>
  <p>{greetMsg}</p>
</div>
