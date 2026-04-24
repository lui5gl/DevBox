<script lang="ts">
  import "./layout.css";

  const { children } = $props();
</script>

<div class="p-4">
  {@render children()}
</div>
