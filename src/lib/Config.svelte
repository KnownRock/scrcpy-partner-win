<script lang="ts">
  import Select, { Option } from "@smui/select";
  import Checkbox from "@smui/checkbox";
  import Dialog, { Header, Title, Content, Actions } from "@smui/dialog";
  import IconButton from "@smui/icon-button";
  import Button, { Label } from "@smui/button";
  import Switch from "@smui/switch";
  // import LoremIpsum from '$lib/LoremIpsum.svelte';
  import Autocomplete from "@smui-extra/autocomplete";
  import FormField from "@smui/form-field";
  import type { Device } from "../utils/devices";
  import { getDevices, lanuchSelf } from "../utils/devices";
  let devices: Device[] = [];
  import Textfield from "@smui/textfield";
  export let open = false;
  export let currentDeviceId = "";
  import LayoutGrid, { Cell } from "@smui/layout-grid";
  let form = getForm(devices);

  type FormItem =
    | {
        type: "option";
        label: string;
        name: string;
        options: { label: string; value: string }[];
        value: string;
        defaultValue?: string;
      }
    | {
        type: "switch";
        label: string;
        name: string;
        value: boolean;
      }
    | {
        type: "text";
        label: string;
        name: string;
        value: string;
      }
    | {
        type: "number";
        label: string;
        name: string;
        value: number;
      }
    | {
        type: "auto";
        label: string;
        name: string;
        value: string;
        options: string[];
      }
    | {
        type: "optional-text";
        label: string;
        name: string;
        value: string;
        enable: boolean;
      }
    | {
        type: "optional-number";
        label: string;
        name: string;
        value: number;
        enable: boolean;
      }
    | {
        type: "optional-option";
        label: string;
        name: string;
        options: { label: string; value: string }[];
        value: string;
        defaultValue?: string;
        enable: boolean;
      }
    | {
        type: "header";
        label: string;
        name: string;
      };

  function getForm(devices: Device[]) {
    const form: FormItem[] = [
      {
        type: "header",
        label: "General",
        name: "general",
      },
      {
        type: "option",
        label: "Device",
        name: "serial",
        options: devices.map((d) => ({ label: d.name, value: d.id })),
        value: currentDeviceId,
      },
      {
        type: "auto",
        label: "Bit Rate",
        name: "bit-rate",
        options: ["32M", "16M", "8M", "4M", "2M", "1M", "512K", "256K"],
        value: "8M",
      },
      {
        type: "auto",
        label: "FPS",
        name: "max-fps",
        options: ["144", "120", "75", "60", "30", "20", "15", "10", "5"],
        value: "60",
      },
      {
        type: "auto",
        label: "Display Buffer",
        name: "display-buffer",
        options: ["100", "50", "30", "20", "10", "5", "0"],
        value: "0",
      },
      {
        type: "optional-number",
        label: "Max Size",
        name: "max-size",
        value: 1080,
        enable: false,
      },
      {
        type: "optional-option",
        label: "Orientation",
        name: "lock-video-orientation",
        options: [
          { label: "Natural orientation", value: "0" },
          { label: "90° counterclockwise", value: "1" },
          { label: "180° counterclockwise", value: "2" },
          { label: "90° clockwise", value: "3" },
        ],
        value: "0",
        enable: false,
      },
      {
        type: "header",
        label: "Screen",
        name: "screen",
      },
      {
        type: "switch",
        label: "Always on top",
        name: "always-on-top",
        value: false,
      },
      {
        type: "switch",
        label: "Fullscreen",
        name: "fullscreen",
        value: false,
      },
      {
        type: "switch",
        label: "Window Borderless",
        name: "window-borderless",
        value: false,
      },
      {
        type: "optional-text",
        label: "Window Title",
        name: "title",
        value: "",
        enable: false,
      },
      {
        type: "optional-text",
        label: "Tcpip",
        name: "tcpip",
        value: "",
        enable: false,
      },
      {
        type: "header",
        label: "Window",
        name: "window",
      },
      {
        type: "optional-number",
        label: "Position X",
        name: "window-x",
        value: 0,
        enable: false,
      },
      {
        type: "optional-number",
        label: "Position Y",
        name: "window-y",
        value: 0,
        enable: false,
      },
      {
        type: "optional-number",
        label: "Width",
        name: "width",
        value: 0,
        enable: false,
      },
    ];

    form.forEach((item) => {
      if (item.type === "option") {
        item.defaultValue = item.value;
      }
    });

    return form;
  }

  let response = "";

  function closeHandler(e: CustomEvent<{ action: string }>) {
    switch (e.detail.action) {
      case "close":
        response = "Closed without response.";
        break;
      case "reject":
        response = "Rejected.";
        break;
      case "accept":
        response = "Accepted.";
        break;
    }
  }

  let updateTime = 0;
  async function setDevices() {
    devices = await getDevices();
    form = getForm(devices);

    updateTime = Date.now();
  }

  async function start() {
    function formToArgs(form: FormItem[]) {
      const args = [];
      for (const item of form) {
        if (item.type === "option") {
          args.push(`--${item.name}=${item.value}`);
        } else if (item.type === "switch") {
          if (item.value) {
            args.push(`--${item.name}`);
          }
        } else if (item.type === "auto") {
          args.push(`--${item.name}=${item.value}`);
        } else if (item.type === "text") {
          args.push(`--${item.name}=${item.value}`);
        } else if (item.type === "number") {
          args.push(`--${item.name}=${item.value}`);
        } else if (item.type === "optional-text") {
          if (item.enable) {
            args.push(`--${item.name}=${item.value}`);
          }
        } else if (item.type === "optional-number") {
          if (item.enable) {
            args.push(`--${item.name}=${item.value}`);
          }
        } else if (item.type === "optional-option") {
          if (item.enable) {
            args.push(`--${item.name}=${item.value}`);
          }
        }
      }

      return args;
    }

    const args = formToArgs(form);
    debugger;
    lanuchSelf(args);
  }

  $: open &&
    (() => {
      setDevices();
    })();
</script>

<Dialog
  bind:open
  fullscreen
  aria-labelledby="fullscreen-title"
  aria-describedby="fullscreen-content"
  on:SMUIDialog:closed={closeHandler}
>
  <Header>
    <Title id="fullscreen-title">Config</Title>
    <IconButton action="close" class="material-icons">close</IconButton>
  </Header>
  <Content id="fullscreen-content">
    <LayoutGrid>
      <!-- {#each Array(9) as _unused, i}
        <Cell>
          <div class="demo-cell">Cell {i + 1}</div>
        </Cell>
      {/each} -->
      {#each form as formItem (formItem.name)}
        {#if formItem.type === "header"}
          <Cell span={12}>
            <h2>{formItem.label}</h2>
          </Cell>
        {:else}
          <Cell>
            {#if formItem.type === "auto"}
              <Autocomplete
                options={formItem.options}
                bind:value={formItem.value}
                label={formItem.label}
              />
            {/if}

            {#if formItem.type === "switch"}
              <FormField align="start">
                <Switch bind:checked={formItem.value} />
                <span slot="label">{formItem.label}</span>
              </FormField>
            {/if}

            {#if formItem.type === "text"}
              <Textfield bind:value={formItem.value} label={formItem.label} />
            {/if}

            {#if formItem.type === "number"}
              <Textfield
                bind:value={formItem.value}
                label={formItem.label}
                type="number"
              />
            {/if}

            {#if formItem.type === "option"}
              <!-- avoid update devices issue -->
              {#key updateTime}
                <Select bind:value={formItem.value} label={formItem.label}>
                  {#each formItem.options as option}
                    <Option value={option.value}>{option.label}</Option>
                  {/each}
                </Select>
              {/key}
            {/if}

            {#if formItem.type === "optional-option"}
              <FormField>
                <Checkbox bind:checked={formItem.enable} />
                <!-- avoid update devices issue -->
                {#key updateTime}
                  <Select
                    slot="label"
                    disabled={!formItem.enable} 
                    bind:value={formItem.value} label={formItem.label}>
                    {#each formItem.options as option}
                      <Option value={option.value}>{option.label}</Option>
                    {/each}
                  </Select>
                {/key}
              </FormField>
            {/if}

            {#if formItem.type === "optional-text"}
              <FormField>
                <Checkbox bind:checked={formItem.enable} />
                <Textfield
                  slot="label"
                  disabled={!formItem.enable}
                  bind:value={formItem.value}
                  label={formItem.label}
                />
              </FormField>
            {/if}

            {#if formItem.type === "optional-number"}
              <FormField>
                <Checkbox bind:checked={formItem.enable} />
                <Textfield
                  slot="label"
                  disabled={!formItem.enable}
                  bind:value={formItem.value}
                  label={formItem.label}
                  type="number"
                />
              </FormField>
            {/if}
          </Cell>
        {/if}
      {/each}
    </LayoutGrid>

    <!-- <div style="height:25em;">
      <Autocomplete
        options={devices.map((d) => d.id)}
        bind:value={currentDeviceId}
        label="Device"
      />
    </div> -->
  </Content>
  <Actions>
    <!-- <Button action="Save">
      <Label>Save</Label>
    </Button> -->
    <Button action="cancel">
      <Label>Cancel</Label>
    </Button>
    <Button action="start" defaultAction on:click={() => start()}>
      <Label>Start</Label>
    </Button>
  </Actions>
</Dialog>

<!-- 
<Button on:click={() => (open = true)}>
  <Label>Open Dialog</Label>
</Button>

<IconButton
  class="material-icons"
  on:click={() => (open = true)}
  title="More options">more_vert
</IconButton>

<pre class="status">Response: {response}</pre>
 -->
