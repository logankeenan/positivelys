//
// Created by Logan Keenan on 7/26/21.
//

import Foundation


class Reminders: Codable {
    var reminders: [Reminder]?

    init(reminders: [Reminder]?) {
        self.reminders = reminders
    }

    enum CodingKeys: String, CodingKey{
        case reminders = "reminders"
    }
}

//TODO: parse this into an list of reminders
// "{"reminders":[{"created_at":"2021-06-06T12:53:19.925917Z","day":"Everyday","hour":12,"id":1,"minute":0,"updated_at":null},{"created_at":"2021-06-06T12:58:33.435521Z","day":"Everyday","hour":12,"id":2,"minute":0,"updated_at":null}]}"
class Reminder: Codable {
    var createdAt, day: String
    var hour, id, minute: Int
    var updatedAt: String?

    enum CodingKeys: String, CodingKey {
        case createdAt = "created_at"
        case day, hour, id, minute
        case updatedAt = "updated_at"
    }

    init(createdAt: String, day: String, hour: Int, id: Int, minute: Int, updatedAt: String?) {
        self.createdAt = createdAt
        self.day = day
        self.hour = hour
        self.id = id
        self.minute = minute
        self.updatedAt = updatedAt
    }
}