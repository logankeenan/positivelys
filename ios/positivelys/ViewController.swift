//
//  ViewController.swift
//  positivelys
//
//  Created by Logan Keenan on 10/30/20.
//
//

import UIKit
import Foundation
import WebKit

protocol ViewControllerDelegate: class {
    func makeAppRequest(_ viewController: ViewController, request_as_json: String)
}

class ViewController: UIViewController, WKUIDelegate, WKScriptMessageHandler, WKNavigationDelegate {

    weak var delegate: ViewControllerDelegate?
    var webView: WKWebView!
    var uri: String = ""
    private var html_markup: String!



    public convenience init(html_markup: String, uri: String) {
        self.init()
        self.html_markup = html_markup
        self.uri = uri
    }

    override func loadView() {
        let webConfiguration = WKWebViewConfiguration()
        webConfiguration.userContentController.add(self, name: "makeAppRequest")
        webView = WKWebView(frame: .zero, configuration: webConfiguration)
        webView.uiDelegate = self
        webView.navigationDelegate = self
        view = webView
    }

    override func viewDidLoad() {
        super.viewDidLoad()

        webView.loadHTMLString(self.html_markup, baseURL: URL(string: AppService.hostName))

    }

    func webView(_ webView: WKWebView, didFinish navigation: WKNavigation!) {
        self.title = webView.title
    }

    public func reload(html_markup: String) {
        self.html_markup = html_markup;
        webView.loadHTMLString(html_markup, baseURL: URL(string: AppService.hostName))
    }

    func userContentController(_ userContentController: WKUserContentController, didReceive message: WKScriptMessage) {
        if message.name == "makeAppRequest" {
            self.delegate?.makeAppRequest(self, request_as_json: message.body as! String)
        }
    }
}
