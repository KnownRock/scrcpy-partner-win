<!-- <div>{JSON.stringify(items)}</div> -->

<div style="
display:flex; padding: 10px; justify-content: space-between;
">

  <div style="border: 1px solid black;">
    
    <div style={`height: ${maxSize[0] * 50}px; width: ${maxSize[1] * 50}px;`}>
      <Grid
        bind:items
        gap={[0, 0]}
        rowHeight={50}
        let:item
        let:dataItem
        cols={[maxSize]}
      >
        <div
          style="
          display: flex; justify-content: center; align-items: center;
          height: 100%; width: 100%; 
          "
        >
          <!-- {JSON.stringify(dataItem)} -->
          <IconButton class="material-icons" on:click={() => 1}>
            crop_square
          </IconButton>
        </div>

      </Grid>

    </div>
  </div>
  <div style="flex: 1; border: 1px solid black; margin-left: 10px;">
    <IconButton class="material-icons" on:click={() => {
      items = [
        ...items,
        {
          6: gridHelp.item({
            x: 0,
            y: 0,
            w: 1,
            h: 1
          }),
          id: uuidv4()
        }
      ]
    }}>
      add
    </IconButton>

  </div>
 
</div>


<script>
  import IconButton from '@smui/icon-button'

  import gridHelp from 'svelte-grid/build/helper/index.mjs'
  import { v4 as uuidv4 } from 'uuid'
  import Grid from 'svelte-grid'
  

  const maxSize = [12, 6]
  // const maxRowLength = maxSize[0]

  let items =
  []
  // [
  //   {
  //     6: gridHelp.item({
  //       x: 0,
  //       y: 0,
  //       w: 1,
  //       h: 1
  //     }),
  //     id: uuidv4()
  //   },
  //   {
  //     6: gridHelp.item({
  //       x: 0,
  //       y: 1,
  //       w: 1,
  //       h: 1
  //     }),
  //     id: uuidv4()
  //   }
  // ]

  $: items && (() => {
    // items[0][6].x = 2
    let maxRowLength = 0

    items.forEach(item => {
      const bottom = item[6].y + item[6].h
      // if (bottom > maxRowLength) {
      //   item[6].y = maxRowLength - item[6].h
      // }
      // maxSize[0] = Math.max(maxSize[0], bottom)
      maxRowLength = Math.max(maxRowLength, bottom)
    })

    maxSize[0] = maxRowLength
    // debugger
  })()
</script>

