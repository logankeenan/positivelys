//
// Created by Logan Keenan on 11/16/20.
//

import UIKit

class ApplicationController: UITabBarController {
    override func viewDidLoad() {
        super.viewDidLoad()

        let positivelys_controller = NavigationController(uri: "\(AppService.hostName)/positivelys")
        positivelys_controller.tabBarItem.title = "Positivelys"

        let reminders_controller = NavigationController(uri: "\(AppService.hostName)/positivelys/new")
        reminders_controller.tabBarItem.title = "Reminders"

        viewControllers = [positivelys_controller, reminders_controller]
    }
}