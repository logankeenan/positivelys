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

    var imagePicker: ImagePicker!
    var imagePickerInputId: String!

    public convenience init(html_markup: String, uri: String) {
        self.init()
        self.html_markup = html_markup
        self.uri = uri
    }

    override func loadView() {
        let webConfiguration = WKWebViewConfiguration()
        webConfiguration.userContentController.add(self, name: "makeAppRequest")
        webConfiguration.userContentController.add(self, name: "invokePhotoPicker")
        webView = WKWebView(frame: .zero, configuration: webConfiguration)
        webView.uiDelegate = self
        webView.navigationDelegate = self
        view = webView

        NotificationCenter.default.addObserver(
                self,
                selector: #selector(appBecomeActive),
                name: UIApplication.willEnterForegroundNotification,
                object: nil
        )
    }

    @objc func appBecomeActive() {
        writeHTMLToFileAndLoad()
    }

    override func viewWillDisappear(_ animated: Bool) {
        super.viewWillDisappear(animated)
        NotificationCenter.default.removeObserver(self, name: UIApplication.willEnterForegroundNotification, object: nil)
    }

    override func viewWillAppear(_ animated: Bool) {
        super.viewWillAppear(animated)
    }

    override func viewDidLoad() {
        super.viewDidLoad()
        writeHTMLToFileAndLoad()


        let attributes = [NSAttributedString.Key.font: UIFont(name: "Nunito-Bold", size: 18)!]
        self.navigationController?.navigationBar.titleTextAttributes = attributes
        self.imagePicker = ImagePicker(presentationController: self, delegate: self)
    }

    func webView(_ webView: WKWebView, didFinish navigation: WKNavigation!) {
        self.title = webView.title
    }

    public func reload(html_markup: String) {
        self.html_markup = html_markup;
        writeHTMLToFileAndLoad()
    }

    func userContentController(_ userContentController: WKUserContentController, didReceive message: WKScriptMessage) {
        if message.name == "makeAppRequest" {
            self.delegate?.makeAppRequest(self, request_as_json: message.body as! String)
        }

        if message.name == "invokePhotoPicker" {
            self.imagePickerInputId = message.body as! String
            self.imagePicker.present()
        }
    }

    private func writeHTMLToFileAndLoad() {
        let paths = FileManager.default.urls(for: .documentDirectory, in: .userDomainMask)
        let documentsDirectory = paths[0]
        let filename = documentsDirectory.appendingPathComponent("index.html")

        do {
            //html is a string
            try self.html_markup.write(to: filename, atomically: true, encoding: String.Encoding.utf8)
        } catch {
            //...
        }
        webView.loadFileURL(filename, allowingReadAccessTo: documentsDirectory)
    }
}

extension ViewController: ImagePickerDelegate {

    func didSelect(image: UIImage?) {
        let temporaryDirectoryURL = URL(fileURLWithPath: NSTemporaryDirectory(),
                isDirectory: true)
        let url = temporaryDirectoryURL.appendingPathComponent("temp-image-picker.png")

        if let data = image?.pngData() {
            do {
                try data.write(to: url)
                self.webView.evaluateJavaScript("window.positivelys.setImagePickerPath('\(url)', '\(self.imagePickerInputId!)')");
            } catch {
                print("Unable to Write Image Data to Disk")
            }
        }
    }
}
