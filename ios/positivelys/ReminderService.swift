//
// Created by Logan Keenan on 7/30/21.
//

import Foundation
import UIKit

class ReminderService {
    var reminders: [Reminder]

    init(reminders: [Reminder]) {
        self.reminders = reminders
    }

    func createAllNotifications() {
        reminders.forEach { reminder in
            let day = reminder.day
            let hour = reminder.hour;
            let minute = reminder.minute;
            let id = reminder.id;
            switch reminder.day {

            case "Everyday":
                createNotification(day: 1, hour: hour, minute: minute, id: id)
                createNotification(day: 2, hour: hour, minute: minute, id: id)
                createNotification(day: 3, hour: hour, minute: minute, id: id)
                createNotification(day: 4, hour: hour, minute: minute, id: id)
                createNotification(day: 5, hour: hour, minute: minute, id: id)
                createNotification(day: 6, hour: hour, minute: minute, id: id)
                createNotification(day: 7, hour: hour, minute: minute, id: id)
            case "Weekdays":
                createNotification(day: 2, hour: hour, minute: minute, id: id)
                createNotification(day: 3, hour: hour, minute: minute, id: id)
                createNotification(day: 4, hour: hour, minute: minute, id: id)
                createNotification(day: 5, hour: hour, minute: minute, id: id)
                createNotification(day: 6, hour: hour, minute: minute, id: id)
            case "Weekends":
                createNotification(day: 1, hour: hour, minute: minute, id: id)
                createNotification(day: 7, hour: hour, minute: minute, id: id)
            case "Sunday":
                createNotification(day: 1, hour: hour, minute: minute, id: id)
            case "Monday":
                createNotification(day: 2, hour: hour, minute: minute, id: id)
            case "Tuesday":
                createNotification(day: 3, hour: hour, minute: minute, id: id)
            case "Wednesday":
                createNotification(day: 4, hour: hour, minute: minute, id: id)
            case "Thursday":
                createNotification(day: 5, hour: hour, minute: minute, id: id)
            case "Friday":
                createNotification(day: 6, hour: hour, minute: minute, id: id)
            case "Saturday":
                createNotification(day: 7, hour: hour, minute: minute, id: id)
            default: break
                    // do nothing
            }
        }
    }

    func createNotification(day: Int, hour: Int, minute: Int, id: Int) {
        let content = UNMutableNotificationContent()
        content.title = "Create Today's Positivelys!"

        var dateComponents = DateComponents()

        // Sunday = 1,  n = 1..7
        dateComponents.weekday = day
        dateComponents.hour = hour
        dateComponents.minute = minute

        let uniqueId = "reminders-\(id)-\(day)-\(hour)-\(minute)"

        let trigger = UNCalendarNotificationTrigger.init(dateMatching: dateComponents, repeats: true)
        let request = UNNotificationRequest(identifier: uniqueId, content: content, trigger: trigger)
        let notificationCenter = UNUserNotificationCenter.current()

        notificationCenter.add(request) { (error) in
            if error != nil {
                // TODO log some error
                print("Unexpected error: \(error).")
            }
        }
    }

    // TODO this probably needs to be adjusted at some point to only remove reminder notifications
    func removeAllNotifications() {
        let notificationCenter = UNUserNotificationCenter.current()
        notificationCenter.removeAllPendingNotificationRequests()
        notificationCenter.removeAllDeliveredNotifications()
    }
}
