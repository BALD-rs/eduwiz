import { Server } from 'socket.io'
import { createServer } from 'http'
const server = createServer()
const io = new Server(server)
io.listen(3000)
