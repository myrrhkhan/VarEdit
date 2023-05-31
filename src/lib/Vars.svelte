<script lang="ts">
	import { invoke } from "@tauri-apps/api/tauri";
	
	async function getPath(): Promise<Map<string, string[]>> {
		let thing: Map<string, string[]> = await invoke('get_vars');
		console.log(thing);
		return thing;
	}
	
	let varsPromise = getPath();
	
	function handleClick() {
		varsPromise = getPath();
	}

	/**
	 * 
	 * @param map
	 * @param key
	 */
	async function getKeys(map: Map<string, string[]>, key: string): Promise<string[]> {
		let thing: string[] = (map!).get(key)!;
		return thing;
	}
	

</script>

<h1>Edit Environment Variables</h1>


{#await varsPromise then allVars}
	{#each (Object.keys(allVars)) as key}
		<h3>{key}</h3>
		{#if allVars}
			<p>something exists</p>
			{#await getKeys(allVars, key) then values}
				console.log("successfully awaited");
				{#each values as value}
					<p>{value}</p>
				{/each}
			{/await}
		{/if}
	{/each}
{/await}
