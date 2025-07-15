import 'package:flutter/material.dart';
import 'package:news_app/model/news.dart';
import 'package:url_launcher/url_launcher.dart';

class NewsDetailPage extends StatelessWidget {
  final News news;

  const NewsDetailPage({super.key, required this.news});

  Future<void> _launchURL(BuildContext context) async {
    final Uri url = Uri.parse(news.link);
    if (await canLaunchUrl(url)) {
      await launchUrl(url);
    } else {
      ScaffoldMessenger.of(context).showSnackBar(
        const SnackBar(content: Text('Could not open the article')),
      );
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text(news.title, maxLines: 1, overflow: TextOverflow.ellipsis),
      ),
      body: Padding(
        padding: const EdgeInsets.all(16.0),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text(news.title, style: Theme.of(context).textTheme.headlineSmall),
            const SizedBox(height: 16),
            Expanded(
              child: SingleChildScrollView(
                child: Text(
                  news.summary?.isNotEmpty == true
                      ? news.summary!
                      : news.description,
                  style: Theme.of(context).textTheme.bodyLarge,
                ),
                // // Flutter - only
                // Text(
                //   news.body,
                //   style: Theme.of(context).textTheme.bodyLarge,
                // ),
              ),
            ),
            const SizedBox(height: 16),
            ElevatedButton(
              onPressed: () => _launchURL(context),
              child: const Text('Read Full Article'),
            ),
          ],
        ),
      ),
    );
  }
}
