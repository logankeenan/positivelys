//
// Created by Logan Keenan on 7/26/21.
//

import Foundation

//TODO: parse this into an list of reminders
// "{"reminders":[{"created_at":"2021-06-06T12:53:19.925917Z","day":"Everyday","hour":12,"id":1,"minute":0,"updated_at":null},{"created_at":"2021-06-06T12:58:33.435521Z","day":"Everyday","hour":12,"id":2,"minute":0,"updated_at":null}]}"
public class Reminder: Codable {
    var id: Int?
    var minute: Int?
    var hour: Int?
    var day: Int?

    enum CodingKeys: String, CodingKey {
        case id
    }
}