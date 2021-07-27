//
// Created by Logan Keenan on 6/6/21.
//

import Foundation

class ResponseHandlers {

    public init(appRequest: AppRequest, appResponse: AppResponse) {

        let reminderCreated = appRequest.uri == "\(AppService.hostName)/reminders" && appRequest.method.lowercased() == "post" && appResponse.status_code == 302
        if reminderCreated {
            let allRemindersRequest = AppRequest(uri: "\(AppService.hostName)/reminders", method: "GET")
            allRemindersRequest.headers["Accept"] = "application/json"

            let allRemindersResponse = AppService().make_request(appRequest: allRemindersRequest)

            // TODO convert response body to reminders class

        }

    }
}