import { getUsername } from '$lib/flow/utils'

export const load = async ({ params }) => {
    return {
        roomCode: params.roomcode,
        username: await getUsername()
    }
}