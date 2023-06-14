<script lang="ts">
	import { invoke } from "@tauri-apps/api/tauri";

	interface variableMap {
		[key: string]: string[];
	}

	// gathers environment variables to display
	async function getPath(): Promise<variableMap> {
		let thing: variableMap = await invoke('get_vars');
		return thing;
	}

	// adds a new environment variable
	async function addVar(key: String, varSubmission: String): Promise<String> {
		let err: String = await invoke('add_var', { key: key, varSubmission: varSubmission});
		showTextBox = false;
		return err;
	}
	
	let varsPromise = getPath(); // promise of map containing all environment variables

	let varSubmission: String;
	let errPromise: String;
	let showTextBox: Boolean = false;

</script>

<h1>Your Computer's Environment Variables:</h1>

{#await varsPromise then allVars}
	{#each Object.keys(allVars) as key}
		{@const values = allVars[key]}
		<h3>{key}</h3>
		{#each values as value}
			<li>{value}</li>
		{/each}
		<button></button>
		<form>
			<input bind:value={varSubmission} type="text">
			<button on:click={() => addVar(key, varSubmission)}>Add Variable</button>
		</form>
		{#await errPromise then err}
			<p>{err}</p>
		{/await}
	{/each}
{/await}