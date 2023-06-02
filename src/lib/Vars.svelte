<script lang="ts">
	import { invoke } from "@tauri-apps/api/tauri";

	interface variableMap {
		[key: string]: string[];
	}

	async function getPath(): Promise<variableMap> {
		let thing: variableMap = await invoke('get_vars');
		return thing;
	}
	
	let varsPromise = getPath();

</script>

<h1>Your Computer's Environment Variables:</h1>

{#await varsPromise then allVars}
	{#each Object.keys(allVars) as key}
		{@const values = allVars[key]}
		<h3>{key}</h3>
		{#each values as value}
			<li>{value}</li>
		{/each}
	{/each}
{/await}