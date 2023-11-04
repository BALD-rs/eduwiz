import { env } from '$env/dynamic/public'
import { config } from '@onflow/fcl'

config({
  'app.detail.title': 'EduWiz',
  'accessNode.api': 'https://rest-testnet.onflow.org',
  'discovery.wallet': 'https://fcl-discovery.onflow.org/testnet/authn',
  'flow.network': 'testnet',
})
