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

	function whileAddingInput(key: String) {
		console.log("box should now show");
		keyBeingEdited = key;
		console.log(key == keyBeingEdited);
	}

	function removeBox() {
		keyBeingEdited = "";
	}

	// adds a new environment variable
	async function addVar(key: String, varSubmission: String): Promise<String> {
		let err: String = await invoke('add_var', { key: key, varSubmission: varSubmission});
		removeBox();
		return err;
	}
	
	let varsPromise = getPath(); // promise of map containing all environment variables

	let keyBeingEdited: String; // key that's being edited
	let varSubmission: String; // environment variable being added
	let errPromise: String; // promise of error message

</script>

<h1>Your Computer's Environment Variables:</h1>

{#await varsPromise then allVars}
	{#each Object.keys(allVars) as key}
		{@const values = allVars[key]}
		<h3>{key}</h3>
		{#each values as value}
			<li>{value}</li>
		{/each}
		<button on:click={() => whileAddingInput(key)}>Add Variable</button>
		{#if key == keyBeingEdited}
			<form>
				<input bind:value={varSubmission} type="text">
				<button on:click={() => addVar(key, varSubmission)}>Add Variable</button>
				<button on:click={removeBox}>Cancel</button>
			</form>
		{/if}
	{/each}
{/await}