<script lang="ts">
  import Details from "./details.svelte";

  let search = $state(localStorage.getItem("search") || "");
  let tokens = $derived(search.split(" "));
  let changed = $state(true);
  let ws_connected = $state(false);
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
    
    if (data.buy_depth_count == 0 && data.sell_depth_count == 0 && !(key in depth_data)) {
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

    socket.addEventListener("open", (event) => {
      ws_connected = true;
    });

    // Event listener for when a message is received from the server
    socket.addEventListener("message", (event) => {
      let data = JSON.parse(event.data);
      data.market_depth_info = data.market_depth_info.slice(0, data.buy_depth_count + data.sell_depth_count);

      add_data(data.token, data);
    });

    // Event listener for when the connection is closed
    socket.addEventListener("close", (event) => {
        // alert("Disconnected from WebSocket server.");
        ws_connected = false;
    });

    // Event listener for when there is an error with the connection
    socket.addEventListener("error", (error) => {
        // alert("WebSocket error:", error);
        ws_connected = false;
    });
  } catch (error) {
    alert(error);
    ws_url = "";
    ws_connected = false;
  };
</script>

<div class="flex justify-center gap-2 my-2">
  {#if ws_connected}
    <span class="bg-blue-100 text-blue-800 text-xs font-medium me-2 px-2.5 flex-wrap content-center rounded dark:bg-blue-900 dark:text-blue-300">
      Connected
    </span>
  {:else}
    <span class="bg-red-100 text-orange-800 text-xs font-medium me-2 px-2.5 flex-wrap content-center rounded dark:bg-blue-900 dark:text-blue-300">
      Disconnected
    </span>
  {/if}
<form class="w-[20%]">
  <div class="relative">
    <div class="absolute inset-y-0 start-0 flex items-center ps-3 pointer-events-none">
        <svg class="w-4 h-4 text-gray-500 dark:text-gray-400" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 20 20">
            <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="m19 19-4-4m0-7A7 7 0 1 1 1 8a7 7 0 0 1 14 0Z"/>
        </svg>
    </div>
    <input bind:value={ws_url} type="search" id="default-search" class="block w-full p-4 ps-10 text-sm text-gray-900 border border-gray-300 rounded-lg bg-gray-50 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="ws://localhost:8080" required />
    <button type="submit" class="absolute inset-y-0 end-0 flex items-center px-4 bg-blue-600 rounded-r-lg text-white">Connect</button>
  </div>
</form>

<form class="w-[20%]">
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
</div>

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