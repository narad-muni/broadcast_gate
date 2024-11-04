<script lang="ts">
  import Details from "./details.svelte";


  let depth_data = $state({});

  function add_data(key, data) {
    if (data.buy_depth_count == 0 && data.sell_depth_count == 0) {
      return
    }

    depth_data[key] = data;
  }

  document.addEventListener("depth_data_event", (event) => {
    add_data(event.detail.token, event.detail);
  })

  // window.webkit.messageHandlers.external.postMessage("Hello")
</script>

<main class="flex flex-wrap">

  {#each Object.keys(depth_data) as key}
    <Details data={depth_data[key]}/>
  {/each}

  {#if Object.keys(depth_data).length == 0}
  <h1>No data to display</h1>
  {/if}
</main>
