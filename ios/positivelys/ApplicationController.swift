//
// Created by Logan Keenan on 11/16/20.
//

import UIKit

class ApplicationController: UINavigationController {
    private func handle_request_response(request: AppRequest) {
        let response = AppService().make_request(appRequest: request)
        let new_uri: String = response.headers!["Content-Location"]!
        let controller = ViewController(html_markup: (response.body)!, uri: new_uri)
        controller.delegate = self

        if (request.method == "POST") {
            //TODO what about POST w/ validation error?

            handlePostRedirectGet(new_uri: new_uri, response: response)
        } else {
            pushViewController(controller, animated: true)
        }
    }

    private func handlePostRedirectGet(new_uri: String, response: AppResponse) {
        var currentController = getCurrentController();

        while (currentController.uri != new_uri) {
            popViewController(animated: true)
            currentController = getCurrentController();
        }

        getCurrentController().reload(html_markup: (response.body)!)
    }

    private func getCurrentController() -> ViewController {
        viewControllers[viewControllers.count - 1] as! ViewController
    }

    override func viewDidLoad() {
        super.viewDidLoad()

        let request = AppRequest(uri: "\(AppService.hostName)/positivelys", method: "GET")
        handle_request_response(request: request)
    }
}

extension ApplicationController: ViewControllerDelegate {
    func makeAppRequest(_ viewController: ViewController, request_as_json: String) {
        let request: AppRequest? = try? JSONDecoder().decode(AppRequest.self, from: request_as_json.data(using: .utf8)!)

        handle_request_response(request: request!)
    }
}