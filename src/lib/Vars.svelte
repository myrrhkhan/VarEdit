<script lang="ts">
	import { invoke } from "@tauri-apps/api/tauri";``
	
	async function getPath(): Promise<Map<string, string[]>> {
		let thing: Map<string, string[]> = await invoke('get_vars');
		console.log(thing);
		return thing;
	}
	
	let varsPromise = getPath();
	
	function handleClick() {
		varsPromise = getPath();
	}

</script>

<h1>Edit Environment Variables</h1>

<button on:click={handleClick}>Show Environment Variables</button>

{#await varsPromise then allVars}
	{#each Array.from(allVars.keys()) as key}
	<p>key</p>
	{/each}
{/await}
