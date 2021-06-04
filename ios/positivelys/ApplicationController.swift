//
// Created by Logan Keenan on 11/16/20.
//

import UIKit

class ApplicationController: UITabBarController {
    override func viewDidLoad() {
        super.viewDidLoad()

        let positivelys_controller = NavigationController(uri: "\(AppService.hostName)/positivelys")
        positivelys_controller.tabBarItem.title = "Positivelys"
        positivelys_controller.tabBarItem.image = UIImage(systemName: "heart.fill")

        let reminders_controller = NavigationController(uri: "\(AppService.hostName)/reminders")
        reminders_controller.tabBarItem.title = "Reminders"
        reminders_controller.tabBarItem.image = UIImage(systemName: "alarm.fill")

        viewControllers = [positivelys_controller, reminders_controller]
    }
}