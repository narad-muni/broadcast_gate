<script>
    export let data;

    import Depth from "./depth.svelte";
    import Ohlc from "./ohlc.svelte";

</script>

<div class="m-1">
    <div class="block max-w-sm rounded-b-none p-3 bg-gray-200 rounded-lg shadow">
        <p class="m-0 text-sm text-black">Token: {data.token}</p>
    </div>
    <div
    class="block rounded-t-none max-w-sm p-4 bg-white border border-gray-200 rounded-lg shadow"
    >
        <Ohlc {data} />
        <Depth {data} />
    </div>
</div>