//
// Created by Logan Keenan on 6/6/21.
//

import Foundation
import UIKit

class RequestResponseMiddleware {
    public func handle(appRequest: AppRequest, appResponse: AppResponse) -> AppResponse {
        handleReminderCreated(appRequest: appRequest, appResponse: appResponse)

        let response = handleNotificationsNotEnabled(appRequest: appRequest, appResponse: appResponse)

        return response
    }

    private func handleNotificationsNotEnabled(appRequest: AppRequest, appResponse: AppResponse) -> AppResponse {
        let requestingRemindersPage = appRequest.uri == "\(AppService.hostName)/reminders" && appRequest.method.lowercased() == "get"
        var response: AppResponse = appResponse;

        if requestingRemindersPage {
            let center = UNUserNotificationCenter.current()
            center.requestAuthorization(options: [.alert, .sound, .badge]) { granted, error in
                if let error = error {
                    // TODO log some error
                }
            }
        }

        return response
    }

    private func handleReminderCreated(appRequest: AppRequest, appResponse: AppResponse) {
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