import configJson from './config.json'

export class AppConfig {
  public friendbot: string = configJson.server.friendbot
  public server: string = configJson.server.url
  public network: string = configJson.server.network
  public showLogs: boolean = configJson.show_logs
}
