//
// Created by Logan Keenan on 6/6/21.
//

import Foundation
import UIKit

class RequestResponseMiddleware {
    public func handle(appRequest: AppRequest, appResponse: AppResponse) {
        handleReminderCreated(appRequest: appRequest, appResponse: appResponse)

        handleNotificationsPrompt(appRequest: appRequest, appResponse: appResponse)
    }

    private func handleNotificationsPrompt(appRequest: AppRequest, appResponse: AppResponse) {
        let requestingRemindersPage = appRequest.uri == "\(AppService.hostName)/reminders" && appRequest.method.lowercased() == "get"

        if requestingRemindersPage {
            let center = UNUserNotificationCenter.current()
            center.requestAuthorization(options: [.alert, .sound, .badge]) { granted, error in
                if let error = error {
                    // TODO log some error
                    print("Unexpected error: \(error).")
                }
            }
        }
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