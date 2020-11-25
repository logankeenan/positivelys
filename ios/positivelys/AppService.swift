//
// Created by Logan Keenan on 11/24/20.
//

import Foundation

public class AppService {

    public static var hostName: String = "https://positivelys.com";

    func appContext() -> String {
        let database_path: String = NSSearchPathForDirectoriesInDomains(.documentDirectory, .userDomainMask, true)[0] + "/database.sqlite"

        let appContext = "{\"database_path\":\"" + database_path + "\"}"
        return appContext
    }

    public func make_request(appRequest: AppRequest) -> AppResponse {
        let appRequestData = try! JSONEncoder().encode(appRequest)
        let appRequestAsJson = String(data: appRequestData, encoding: .utf8)

        let result = make_app_request(appRequestAsJson, appContext())
        let app_response_as_json = String(cString: result!)

        make_app_request_free(UnsafeMutablePointer(mutating: result))

        let response: AppResponse? = try? JSONDecoder().decode(AppResponse.self, from: app_response_as_json.data(using: .utf8)!)

        if response?.status_code == 302 {
            let location: String? = response?.headers?["Location"]
            let request = AppRequest(uri: "\(AppService.hostName)\(location!)", method: "GET")
            return make_request(appRequest: request)
        }

        if ((response?.headers) == nil) {
            response?.headers = Dictionary<String, String>()
            response?.headers!["Content-Location"] = appRequest.uri
        }
        return response!;
    }

}