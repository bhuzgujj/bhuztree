<script lang="ts">
	export let title: string
</script>

<div class="bg-4 rounded-md p-1">
    <h2>{title}</h2>
    <table class="flex flex-col bg-5 rounded-md">
        <slot/>
    </table>
</div>