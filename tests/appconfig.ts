import configJson from './config.json'

export class AppConfig {
  public friendbot: string = ''
  public server: string = ''
  public network: string = ''
  public showLogs: boolean = true

  public parseConfig() {
    this.friendbot = configJson.server.friendbot
    this.server = configJson.server.url
    this.network = configJson.server.network
    this.showLogs = configJson.show_logs
  }
}
