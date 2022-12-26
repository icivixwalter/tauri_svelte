<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import Tabella from "./Tabella.svelte";
    let count = 0;

    let rows : string[] = [];
    let mesg = "";
    async function connettiaccess() {
        console.log("Chiamata ad acces via Rust");
        rows = await invoke("connettiaccess");
        count += 1;
    }
    async function greet() {
        console.log("X");
        mesg = await invoke("greet");
    }
</script>

<button on:click={connettiaccess}>
    click {mesg} {count}
</button>
<Tabella rows={rows}/>
