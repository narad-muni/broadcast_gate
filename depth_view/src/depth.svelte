<script>
    export let data;

    let buy_count = data.buy_depth_count;
    let sell_count = data.sell_depth_count;

    let buy = data.market_depth_info.slice(0, buy_count);
    let sell = data.market_depth_info.slice(buy_count, buy_count + sell_count);

    let max_buy = buy.reduce((a, b) => Math.max(a, b.qty), 0);
    let max_sell = sell.reduce((a, b) => Math.max(a, b.qty), 0);
</script>

{#snippet depth(depth)}
{@const bg = depth.bid ? 'bg-blue-100' : 'bg-depth-100'}
{@const width = (depth.qty*100/depth.max)}

<tr>
    <td>{depth.price}</td>
    <td class="text-end px-2">{depth.number_of_orders}</td>
    <td class="text-end flex justify-end">
        <div class="flex duration-300 justify-end w-[{width}%] {bg}">
            {depth.qty}
        </div>
    </td>
</tr>
{/snippet}

<div class="flex gap-x-2">
    <table class="w-full table-fixed h-min text-blue-500">
        <thead class="text-xs text-gray-500">
            <tr>
                <th class="font-thin text-start">BID</th>
                <th class="font-thin text-start">ORDERS</th>
                <th class="font-thin text-end">QTY</th>
            </tr>
        </thead>
        <tbody>
        {#each buy as buy_depth}
            {@render depth({
                ...buy_depth,
                bid: true,
                max: max_buy,
            })}
        {/each}
        </tbody>
    </table>
    <div class="bg-gray-400 w-[1px]">&nbsp;</div>
    <table class="w-full table-fixed h-min text-red-500">
        <thead class="text-xs text-gray-500">
            <tr>
                <th class="font-thin text-start">ASK</th>
                <th class="font-thin text-start">ORDERS</th>
                <th class="font-thin text-end">QTY</th>
            </tr>
        </thead>
        <tbody>
        {#each sell as sell_depth}
            {@render depth({
                ...sell_depth,
                max: max_sell,
            })}
        {/each}
        </tbody>
    </table>
</div>
