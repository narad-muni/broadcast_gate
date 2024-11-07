<script lang="ts">
  import Details from "./details.svelte";

  let search = $state(localStorage.getItem("search") || "");
  let tokens = $derived(search.split(" "));
  let changed = $state(true);
  let depth_data = $state({});
  let ws_url = $state(localStorage.getItem("ws_url") || "ws://localhost:8080");

  $effect(() => {
    localStorage.setItem("search", search);
  });

  $effect(() => {
    localStorage.setItem("ws_url", ws_url);
  });

  function add_data(key, data) {
    console.log("Received ", key);

    key = key.toString();
    
    if (data.buy_depth_count == 0 && data.sell_depth_count == 0) {
      return
    }

    if (!tokens.includes(key) && search != "") {
      return
    }

    depth_data[key] = data;

    changed = !changed;

    if (key == "434648" || key == 434648) {
      console.log(depth_data);
    }
  }

  document.addEventListener("depth_data_event", (event) => {
    add_data(event.detail.token, event.detail);
  });

  let socket = {};
  try {
    socket = new WebSocket(ws_url);

    // Event listener for when a message is received from the server
    socket.addEventListener("message", (event) => {
      let data = JSON.parse(event.data);
      data.market_depth_info = data.market_depth_info.slice(0, data.buy_depth_count + data.sell_depth_count);

      add_data(data.token, data);

      // console.log(data);
    });

    // Event listener for when the connection is closed
    socket.addEventListener("close", (event) => {
        alert("Disconnected from WebSocket server.");
    });

    // Event listener for when there is an error with the connection
    socket.addEventListener("error", (error) => {
        alert("WebSocket error:", error);
    });
  } catch (error) {
    alert(error);
    ws_url = "";
  };
</script>

<form class="max-w-md mx-auto">   
  <div class="relative">
      <div class="absolute inset-y-0 start-0 flex items-center ps-3 pointer-events-none">
          <svg class="w-4 h-4 text-gray-500 dark:text-gray-400" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 20 20">
              <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="m19 19-4-4m0-7A7 7 0 1 1 1 8a7 7 0 0 1 14 0Z"/>
          </svg>
      </div>
      <input bind:value={ws_url} type="search" id="default-search" class="block w-full p-4 ps-10 text-sm text-gray-900 border border-gray-300 rounded-lg bg-gray-50 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="ws://localhost:8080" required />
      <button type="submit" class="absolute inset-y-0 end-0 flex items-center pe-4">Connect</button>
  </div>
</form>

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


{#key changed}
{#each Object.entries(depth_data) as [key, data]}
{console.log(tokens.includes(key), tokens, key)}
    {#if tokens.includes(key) || search == ""}
    <Details {data}/>
    {/if}
{/each}

{#if Object.keys(depth_data).length == 0}
<h1>No data to display</h1>
{/if}
{/key}
</main>