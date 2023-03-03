const net = require('net')

const PIPE_NAME = 'ppppppppppppp'
const PIPE_PATH = '\\\\.\\pipe\\' + PIPE_NAME
const L = console.log
// == Client part == //
const client = net.connect(PIPE_PATH, function () {
  L('Client: on connection')
})
client.on('data', function (data) {
  L('Client: on data:', data.toString())
  client.end()
})
client.write(JSON.stringify({ arg_json: { data: { name: 'test1' } }, func: 'create', table: 'test' }))

client.end()
