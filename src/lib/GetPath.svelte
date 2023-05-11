<script lang="ts">
	import { invoke } from "@tauri-apps/api/tauri";


	let allVars = new Map<string[], string>();
	const tbl = document.createElement("table");
	
	async function getPath() {
		allVars = await invoke('getVars'); // get variables
		allVars.forEach((key: string, value: string[]) => {
			// for each variable, make a table and put the stuff in the table
			let numRows = value.length;


			for (let i = 0; i < numRows; i++) {

				// make a row
				const row = document.createElement("tr");
				// add to rows
				for (let j = 0; j < 2; j++) {
					if (i == 0 && j == 0) {
						const cell = document.createElement("td");
						cell.appendChild(document.createTextNode(key));
						cell.colSpan = numRows;
						row.appendChild(cell);
					} else if (j == 1) {
						const cell = document.createElement("td");
						cell.appendChild(document.createTextNode(value[i]));
						row.appendChild(cell);
					}
				}
			}
		})

	}

</script>