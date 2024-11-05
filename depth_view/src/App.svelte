<script lang="ts">
  import Details from "./details.svelte";

  let search = $state("");
  let tokens = $derived(search.split(" "));
  let depth_data = $state({});

  function add_data(key, data) {
    key = key.toString();
    
    if (data.buy_depth_count == 0 && data.sell_depth_count == 0) {
      return
    }

    if (!tokens.includes(key) && search != "") {
      return
    }

    depth_data[key] = data;
  }

  document.addEventListener("depth_data_event", (event) => {
    add_data(event.detail.token, event.detail);
  })

  // window.webkit.messageHandlers.external.postMessage("Hello")
</script>

<form class="max-w-md mx-auto">   
  <label for="default-search" class="mb-2 text-sm font-medium text-gray-900 sr-only dark:text-white">Search</label>
  <div class="relative">
      <div class="absolute inset-y-0 start-0 flex items-center ps-3 pointer-events-none">
          <svg class="w-4 h-4 text-gray-500 dark:text-gray-400" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 20 20">
              <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="m19 19-4-4m0-7A7 7 0 1 1 1 8a7 7 0 0 1 14 0Z"/>
          </svg>
      </div>
      <input bind:value={search} type="search" id="default-search" class="block w-full p-4 ps-10 text-sm text-gray-900 border border-gray-300 rounded-lg bg-gray-50 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="429390 429392" required />
  </div>
</form>
<main class="flex flex-wrap justify-evenly">

  {#each Object.keys(depth_data) as key}
    {#if tokens.includes(key) || search == ""}
      <Details data={depth_data[key]}/>
    {/if}
  {/each}

  {#if Object.keys(depth_data).length == 0}
  <h1>No data to display</h1>
  {/if}
</main>
