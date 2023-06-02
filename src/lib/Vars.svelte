<script lang="ts">
	import { invoke } from "@tauri-apps/api/tauri";
	
	async function getPath(): Promise<Map<string, string[]>> {
		let thing: Map<string, string[]> = await invoke('get_vars');
		return thing;
	}
	
	let varsPromise = getPath();

</script>

<h1>Edit Environment Variables</h1>

{#await varsPromise then allVars}
	{#each [...allVars] as [key, vals]}
		<h3>Values for {key}</h3>
		{#each vals as val}
			<li>{val}</li>
		{/each}
	{/each}
{/await}