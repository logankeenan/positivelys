//
// Created by Logan Keenan on 6/6/21.
//

import Foundation

class ResponseHandlers {

    public init(appRequest: AppRequest, appResponse: AppResponse) {

        // TODO needs to handle when a reminder is removed.
        let reminderCreated = appRequest.uri == "\(AppService.hostName)/reminders" && appRequest.method.lowercased() == "post" && appResponse.status_code == 302
        if reminderCreated {
            let allRemindersRequest = AppRequest(uri: "\(AppService.hostName)/reminders", method: "GET")
            allRemindersRequest.headers!["Accept"] = "application/json"

            let allRemindersResponse = AppService().make_request(appRequest: allRemindersRequest)

            do {
                let reminders: Reminders? = try JSONDecoder().decode(Reminders.self, from: (allRemindersResponse.body?.data(using: .utf8))!)
                let reminderService = ReminderService(reminders: (reminders?.reminders)!)
                reminderService.removeAllNotifications()
                reminderService.createAllNotifications()
            } catch {
                // TODO log some error
                print("Unexpected error: \(error).")
            }


        }

    }
}