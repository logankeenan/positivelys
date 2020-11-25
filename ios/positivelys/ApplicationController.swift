//
// Created by Logan Keenan on 11/16/20.
//

import UIKit

// TODO
//  if the request is a post and the response has the previous page then
//  pop two views (the form that posted, and the previous) and push the new view on
class ApplicationController: UINavigationController {
    override func viewDidLoad() {
        super.viewDidLoad()
        let request = AppRequest(uri: "\(AppService.hostName)/positivelys", method: "GET")
        let response = AppService().make_request(appRequest: request)
        let controller = ViewController(html_markup: (response.body)!)
        controller.delegate = self

        pushViewController(controller, animated: true)
    }
}

extension ApplicationController: ViewControllerDelegate {
    func makeAppRequest(_ viewController: ViewController, request_as_json: String) {
        let request: AppRequest? = try? JSONDecoder().decode(AppRequest.self, from: request_as_json.data(using: .utf8)!)
        let response = AppService().make_request(appRequest: request!)
        let controller = ViewController(html_markup: (response.body)!)
        controller.delegate = self
        pushViewController(controller, animated: true)
    }
}