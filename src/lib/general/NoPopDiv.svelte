<!-- svelte-ignore a11y-click-events-have-key-events -->
<div on:click={e => {
  e.stopPropagation()
  e.preventDefault()
}} 
{...$$restProps}
>
  <slot></slot>
</div>
