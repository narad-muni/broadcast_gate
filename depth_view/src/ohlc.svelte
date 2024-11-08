<script>
    export let data;

    let ltt = new Date(data.ltt * 1000);
    ltt = ltt.toLocaleString();

    let lut = new Date(data.lut / 1000000);
    lut = lut.toLocaleString();
</script>

<div class="grid grid-flow-col grid-rows-4 gap-1 justify-between mb-3 text-xs text-gray-700">
    <span><span class="text-black">Open:</span> {data.open_price/100}</span>
    <span><span class="text-black">High:</span> {data.high_price/100}</span>
    <span><span class="text-black">Low:</span> {data.low_price/100}</span>
    <span><span class="text-black">Close:</span> {data.close_price/100}</span>
    <span><span class="text-black">Ltt:</span> {ltt}</span>
    <span><span class="text-black">Ltp:</span> {data.ltp/100}</span>
    <span><span class="text-black">Ltq:</span> {data.ltq}</span>
    <span><span class="text-black">Lut:</span> {lut}</span>
    <span><span class="text-black">Atp:</span> {data.atp/100}</span>
    <span><span class="text-black">Vtt:</span> {data.volume_traded_today}</span>
</div>