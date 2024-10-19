<script>
  import { VERSION } from "$lib/stores";
</script>

<div>
    <slot></slot>
    <p>{VERSION}</p>
</div>

<style lang="scss">
    p {
      position: fixed;
      bottom: 0;
      right: 0;
      margin: 0;
      padding: 10px 10px 8px;
      font-family: "DM Sans", sans-serif;
      font-size: 15px;
      color: gray;
    }
</style>