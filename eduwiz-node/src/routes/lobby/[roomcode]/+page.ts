export const load = ({ params }) => {
    return {
        roomCode: params.roomcode
    }
}

export interface RoomCode {
    roomCode: string
}