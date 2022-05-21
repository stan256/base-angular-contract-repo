import {Injectable} from '@angular/core'
import {ConnectConfig, Contract, Near, WalletConnection} from "near-api-js"
import * as nearAPI from "near-api-js"

@Injectable({
  providedIn: 'root'
})
export class NearService {
  near: Near | undefined = undefined
  wallet: WalletConnection | undefined = undefined
  auctionContract: any


  public async init() {
    await nearAPI.connect(config).then(near => {
      this.near = near
      this.wallet = new WalletConnection(near, null)
      this.auctionContract = new Contract(
        this.wallet.account(),
        CONTRACT,
        {
          changeMethods: [],
          viewMethods: []
        }
      )
    })
  }


  signIn() {
    return this.wallet!.requestSignIn(CONTRACT, "Charitable auction - Junkwin")
  }

  signOut() {
    this.wallet!.signOut()
  }

  authenticated(): boolean {
    if (this.wallet) {
      return this.wallet!.isSignedIn()
    } else {
      return false
    }
  }
}

const CONTRACT: string = "contract-id"
const GAS = "300000000000000";

let config: ConnectConfig = {
  networkId: "testnet",
  keyStore: new nearAPI.keyStores.BrowserLocalStorageKeyStore(),
  nodeUrl: "https://rpc.testnet.near.org",
  walletUrl: "https://wallet.testnet.near.org",
  helperUrl: "https://helper.testnet.near.org"
}
