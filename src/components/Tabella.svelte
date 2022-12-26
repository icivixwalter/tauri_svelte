<script lang="ts">
    export let rows;
</script>

<table border="1">
    <thead>
    <th>Id</th>
    <th>Nome Cognome</th>
    <th>Nome</th>
    <th>Cognome</th>
    <th>Abbreviazione</th>
    </thead>
    {#each rows as row}
        <tr>
            {#each row.split(";") as cell}
                <td>{cell}</td>
            {/each}
        </tr>
    {/each}
</table>

<style>
    table {
        border: 2px;
    }
</style>
